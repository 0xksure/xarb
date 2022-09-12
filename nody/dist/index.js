import LIFI from '@lifi/sdk';
const main = () => {
    const lifiConfig = {
        apiUrl: 'https://staging.li.quest/v1/',
    };
    const sdk = new LIFI(lifiConfig);
    // get tools
    //   const tools = await sdk.getTools();
    //   console.log('tools: ', tools);
    //   const chains = await sdk.getChains();
    //   console.log('chains: ', chains);
    //   const tokens = await sdk.getTokens();
    //   console.log('tokens: ', tokens);
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
