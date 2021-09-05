const assert = await import("assert");
const anchor = await import("@project-serum/anchor");

const serumDevnetKeyStr = "DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY";
const devnetClusterUrl = "https://api.devnet.solana.com";

const serumDevnetKey = new anchor.web3.PublicKey(serumDevnetKeyStr);

const provider = anchor.Provider.local(devnetClusterUrl);

anchor.setProvider(provider);

const connection = provider.connection;

const version = await connection.getVersion();

console.log(version);

const accountInfo = await connection.getAccountInfo(serumDevnetKey);

console.log(accountInfo);
