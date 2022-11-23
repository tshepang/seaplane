import Identify from './identify';

import { log } from '../logging';
import { HTTPError } from '../model/errors';

export const headers = (apiKey: string) => ({
  Accept: 'application/json',
  'Content-Type': 'application/json',
  Authorization: `Bearer ${apiKey}`,
});

export default class Request {
  private identify: Identify;

  constructor(identify: Identify) {
    this.identify = identify;
  }

  private async renewIfFails(error: any, request: (token: string) => Promise<any>): Promise<any> {
    const httpError = new HTTPError(error.response.status, JSON.stringify(error.response.data));

    if (error.response.status != 401 || !this.identify.autoRenew) {
      throw httpError;
    }

    log.info('Auto-Renew, renewing the token...');
    const token = await this.identify.renewToken();
    return await request(token);
  }

  async send(request: (token: string) => Promise<any>): Promise<any> {
    const accessToken: string = this.identify.accessToken || (await this.identify.getToken());

    try {
      const result = await request(accessToken);
      
      return result.data;
    } catch (err: any) {    // eslint-disable-line        
      const title = err?.response?.data?.title || "No title"
      const detail = err?.response?.data?.detail || "No detail"      
      log.error(`Request error: ${title} ${detail}`);
      return await this.renewIfFails(err, request);
    }
  }
}
