declare type ConfigValues = {
    apiKey: string;
    identifyEndpoint?: string;
    computeEndpoint?: string;
    coordinationEndpoint?: string;
    autoRenew?: boolean;
    accessToken?: string;
};
export default class Configuration {
    apiKey: string;
    identifyEndpoint: string;
    computeEndpoint: string;
    coordinationEndpoint: string;
    autoRenew: boolean;
    accessToken?: boolean;
    constructor(configValues: ConfigValues);
    values(): ConfigValues;
}
export {};
//# sourceMappingURL=configuration.d.ts.map