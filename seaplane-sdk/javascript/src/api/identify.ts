const axios = require('axios'); // eslint-disable-line
 
import Configuration from '../configuration';
import { headers } from './request';
import { log } from '../logging';
import { HTTPError } from '../model/errors';

export default class Identify {
  url: string;
  accessToken?: string;
  apiKey?: string;
  autoRenew: boolean;

  constructor(configuration: Configuration) {
    this.url = `${configuration.values().identifyEndpoint}/identity/token`;
    this.apiKey = configuration.apiKey;
    this.accessToken = configuration.accessToken;
    this.autoRenew = configuration.autoRenew;
  }

  setUrl(url: string) {
    this.url = url;
  }

  setToken(accessToken: string) {
    this.autoRenew = false;
    this.accessToken = accessToken;
  }

  async getToken(): Promise<string> {
    try {
      log.info('Requestiong access token...');
      const json = {};
      const result = await axios.post(this.url, json, {
        headers: headers(this.apiKey || ''),
      });

      this.accessToken = result.data.token;
      return result.data.token;
    } catch (err: any) { // eslint-disable-line
      if (!this.apiKey) {
        log.error('API KEY not set, use sea.config.setApiKey');
      } else {
        this.accessToken = undefined;
        log.error(`Request access token exception with code ${err.response.status}, error ${err.data}`);
      }
      throw new HTTPError(err.response.status, err.data);
    }
  }

  async renewToken(): Promise<string> {
    this.accessToken = await this.getToken();
    return this.accessToken;
  }
}
