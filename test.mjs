const assert = await import("assert");
const anchor = await import("@project-serum/anchor");
const serum = await import("@project-serum/serum");

const web3 = anchor.web3;

const Account = web3.Account;
const Connection = web3.Connection;
const PublicKey = web3.PublicKey;
const Market = serum.Market;

const devnetClusterUrl = "https://api.devnet.solana.com";
const mainnetClusterUrl = "https://api.mainnet-beta.solana.com";

const serumDevnetKeyStr = "DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY";
const serumDevnetKey = new PublicKey(serumDevnetKeyStr);

const serumMainnetKeyStr = "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin";
const serumMainnetKey = new PublicKey(serumMainnetKeyStr);

const solUsdcMainnetMarketStr = "9wFFyRfZBsuAha4YcuxcXLKwMxJR43S7fPfQLusDBzvT";
const solUsdcMainnetMarket = new PublicKey(solUsdcMainnetMarketStr);

const clusterUrl = mainnetClusterUrl;
const serumProgramAddress = serumMainnetKey;
const marketAddress = solUsdcMainnetMarket;

const provider = anchor.Provider.local(clusterUrl);

anchor.setProvider(provider);

const connection = provider.connection;

const market = await Market.load(connection, marketAddress, {}, serumProgramAddress);

// Fetching orderbooks
let bids = await market.loadBids(connection);
let asks = await market.loadAsks(connection);
// L2 orderbook data
for (let [price, size] of bids.getL2(20)) {
    console.log(price, size);
}
// Full orderbook data
for (let order of asks) {
    console.log(
        order.orderId,
        order.price,
        order.size,
        order.side, // 'buy' or 'sell'
    );
}






/*
const version = await connection.getVersion();

console.log(version);

const accountInfo = await connection.getAccountInfo(serumDevnetKey);

console.log(accountInfo);
*/
