import { encode } from 'url-safe-base64';
const axios = require('axios');
export default class Metadata {
    url;
    configuration;
    constructor(configuration) {
        this.url = `${configuration.values().coordinationEndpoint}/config`;
        this.configuration = configuration;
    }
    async set(keyValue) {
        const url = `${this.url}/base64:${encode(keyValue.key)}`;
        axios.put(this.url, encode(keyValue.value), {
            headers: {
                "Accept": "application/json",
                "Content-Type": "application/json",
                "Authorization": `Bearer ${this.configuration.apiKey}`,
            }
        });
    }
}
