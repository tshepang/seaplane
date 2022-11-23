import Configuration from "../configuration";
export declare type KeyValue = {
    key: string;
    value: string;
};
export default class Metadata {
    url: string;
    configuration: Configuration;
    constructor(configuration: Configuration);
    set(keyValue: KeyValue): Promise<void>;
}
//# sourceMappingURL=metadata.d.ts.map