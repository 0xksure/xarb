"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const sdk_1 = __importDefault(require("@lifi/sdk"));
const chainAnalysis = (chains, tokens) => {
    console.log('=== Chain analysis ====');
    console.log('> Number of chains: ', chains.length);
    const tokenHash = {};
    chains.forEach((chain) => {
        tokens.tokens[chain.id].forEach((token) => {
            if (token.name in tokenHash) {
                tokenHash[token.name] = {
                    occur: tokenHash[token.name].occur + 1,
                    chainNames: tokenHash[token.name].chainNames.concat(chain.name),
                    prices: tokenHash[token.name].prices.concat(token.priceUSD),
                };
            }
            else {
                tokenHash[token.name] = {
                    occur: 1,
                    chainNames: [chain.name],
                    prices: [token.priceUSD],
                };
            }
        });
    });
    chains.forEach((chain, idx) => {
        const printTokens = tokens.tokens[chain.id]
            .filter((token) => token.symbol.includes('BUSD'))
            .map((token) => {
            return `- name: ${token.name} , price: $${token.priceUSD}, ${token.symbol}`;
        })
            .join(' \n');
        const print = ` > ${idx} ${chain.name}: coin = ${chain.coin}, type = ${chain.chainType}, tokens: ${printTokens}`;
        console.log(print);
    });
    console.log('=== Tokens ===');
    console.log(Object.entries(tokenHash).forEach((token) => {
        if (token[1].occur > 5) {
            console.log(`${token[0]} \n chain | price`);
            const prrr = token[1].chainNames.map((name, idx) => {
                return `${name}  ${token[1].prices[idx]}`;
            });
            console.log(prrr);
        }
    }));
};
const exchangeAnalysis = (tools) => {
    console.log('=== Exchange Analysis ===');
    console.log('> number of exchanges: ', tools.exchanges.keys.length);
    const exchanges = tools.exchanges;
    exchanges.forEach((exc, idx) => {
        console.log(' > ', idx, ': name = ', exc.name, ' key = ', exc.key);
    });
};
const main = async () => {
    const lifiConfig = {
        apiUrl: 'https://staging.li.quest/v1/',
    };
    console.log(sdk_1.default);
    const sdk = new sdk_1.default();
    // get tools
    const tools = await sdk.getTools();
    exchangeAnalysis(tools);
    const tokens = await sdk.getTokens();
    //console.log('tokens: ', tokens.tokens['1']);
    const chains = await sdk.getChains();
    chainAnalysis(chains, tokens);
    // get routes
    //   const routeOptions: RouteOptions = {
    //     order: 'RECOMMENDED',
    //   };
    //   const routesRequest: RoutesRequest = {
    //     options: routeOptions,
    //   };
    //   const routes = await sdk.getRoutes();
};
main();
