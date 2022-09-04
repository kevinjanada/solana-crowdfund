import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
  AccountInfo,
} from "@solana/web3.js";
import * as fs from "fs";
import * as os from "os";
import { serialize as borshSerialize, deserialize } from "borsh";
import * as BN from "bn.js";
import { Buffer } from "buffer";
import * as util from "util";

/*
const encodeFundName = (name: string) => {
  const NAME_MAX_LENGTH = 256;
  return encodeString(name, NAME_MAX_LENGTH);
}

const encodeString = (str: string, maxLen: number) => {
  const strSizeArrBuff = new ArrayBuffer(4);
  const strSizeView = new DataView(strSizeArrBuff);
  strSizeView.setUint32(0, str.length, true);
  const strSizeU8Arr = new Uint8Array(strSizeView.buffer);

  const strU8Arr = new Uint8Array(maxLen);
  const strU8Src = new util.TextEncoder().encode(str);
  strU8Arr.set(strU8Src);

  const encoded = new Uint8Array(4 + maxLen);
  encoded.set(strSizeU8Arr);
  encoded.set(strU8Arr, strSizeU8Arr.length);

  return encoded;
}
*/

const decodeString = (u8Arr: Uint8Array) => {
  const strLengthArr = u8Arr.slice(0, 4);
  const strLengthView = new DataView(strLengthArr.buffer, 0);
  const strLength = strLengthView.getUint32(0, true);
  const strContentArr = u8Arr.slice(4, 4 + strLength);
  return new util.TextDecoder().decode(strContentArr);
}

const homedir = os.homedir();
const privateKeyPath = homedir + "/.config/solana/devnet.json"

const getPrivateKey = () => {
  return Uint8Array.from(
    JSON.parse(fs.readFileSync(privateKeyPath) as unknown as string)
  )
}

const getKeypair = () => {
  const privateKey = getPrivateKey();
  const keypair = Keypair.fromSecretKey(privateKey);
  return keypair;
}

export class CrowdfundAccountData {
  public is_initialized: boolean;
  public name: Uint8Array;
  public initializer_pubkey: PublicKey;
  public goal_amount: BN;
  public deadline: BN;
  public bump: number;

  constructor({
    is_initialized,
    name,
    initializer_pubkey,
    goal_amount,
    deadline,
    bump
  }: {
    is_initialized: boolean,
    name: Uint8Array,
    initializer_pubkey: PublicKey,
    goal_amount: BN,
    deadline: BN,
    bump: number,
  }) {
    this.is_initialized = is_initialized;
    this.name = name;
    this.initializer_pubkey = initializer_pubkey;
    this.goal_amount = goal_amount;
    this.deadline = deadline;
    this.bump = bump;
  }

  static schema: any = new Map([
    [
      CrowdfundAccountData,
      {
        kind: "struct",
        fields: [
          ["is_initialized", "u8"],
          ["name", [260]],
          ["initializer_pubkey", [32]],
          ["goal_amount", "u64"],
          ["deadline", "u64"],
          ["bump", "u8"],
        ],
      },
    ]
  ]);

  display() {
    const is_initialized = Boolean(this.is_initialized);
    const name = decodeString(this.name);
    const initializer_pubkey = new PublicKey(this.initializer_pubkey).toBase58();
    const goal_amount = this.goal_amount.toString();
    const deadline = this.deadline.toString();
    return {
      is_initialized,
      name,
      initializer_pubkey,
      goal_amount,
      deadline,
    }
  }
}

class CreateFundPayload {
  public name: string;
  public goal_amount: BN;
  public deadline: BN;

  constructor(
    { name, goal_amount, deadline }: { name: string, goal_amount: BN, deadline: BN }
  ) {
    this.name = name;
    this.goal_amount = goal_amount;
    this.deadline = deadline;
  }

  serialize() {
    const schema = new Map([
      [
        CreateFundPayload,
        { kind: "struct",
          fields: [
            ["name", "string"],
            ["goal_amount", "u64"],
            ["deadline", "u64"],
          ],
        },
      ],
    ]);

    return borshSerialize(schema, this);
  }
}

const createFund = async () => {
  const crowdfundProgramId = new PublicKey("BveZUHmtftCxRYvUgaZiwtSrQ9uVo6siqBE2aLDxPLpX");
  const keypair = getKeypair();
  const [crowdfundAccount] = await PublicKey.findProgramAddress(
    [Buffer.from("crowdfund"), keypair.publicKey.toBuffer()],
    crowdfundProgramId
  )

  const connection = new Connection("http://localhost:8899", "confirmed");

  const payload = new CreateFundPayload({
    name: "my crowdfund",
    goal_amount: new BN(10),
    deadline: new BN(1661807720),
  });
  const payloadSerialized = payload.serialize();
  const ixIndex = Uint8Array.of(0);

  const dataArray = new Uint8Array(ixIndex.length + payloadSerialized.length);
  dataArray.set(ixIndex);
  dataArray.set(payloadSerialized, ixIndex.length);

  const createFundIx = new TransactionInstruction({
    programId: crowdfundProgramId,
    keys: [
      { pubkey: keypair.publicKey, isSigner: true, isWritable: false },
      { pubkey: crowdfundAccount, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    data: Buffer.from(dataArray),
  });

  const tx = new Transaction().add(createFundIx);

  await connection.sendTransaction(
    tx,
    [keypair],
    { skipPreflight: false, preflightCommitment: "confirmed" },
  );

  const crowdfundAccountInfo = await connection.getAccountInfo(crowdfundAccount) as AccountInfo<Buffer>;

  const crowdfundData = deserialize(
    CrowdfundAccountData.schema,
    CrowdfundAccountData,
    crowdfundAccountInfo.data
  );

  console.log({ display: crowdfundData.display() });
}

createFund();
