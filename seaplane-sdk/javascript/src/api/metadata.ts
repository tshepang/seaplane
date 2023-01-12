import Configuration from '../configuration';
import Request from './request';
import seaFetch from './seaFetch';
import { Key, KeyValue, KeyValuePage, mapToKeyValue, mapToKeyValuePage } from '../model/metadata';

import { encode } from '../utils/base64';

export default class Metadata {
  url: string;
  request: Request;

  constructor(configuration: Configuration) {
    this.url = `${configuration.values().coordinationEndpoint}/config`;
    this.request = new Request(configuration.identify);
  }

  async set(keyValue: KeyValue): Promise<boolean> {
    const url = `${this.url}/base64:${encode(keyValue.key)}`;
    const data = encode(keyValue.value);

    const result = await this.request.send((token) => seaFetch(token).put(url, data));

    return result === 'Ok';
  }

  async get(key: Key): Promise<KeyValue> {
    const url = `${this.url}/base64:${encode(key.key)}`;

    const result = await this.request.send((token) => seaFetch(token).get(url));

    return mapToKeyValue(result);
  }

  async delete(key: Key): Promise<boolean> {
    const url = `${this.url}/base64:${encode(key.key)}`;

    const result = await this.request.send((token) => seaFetch(token).delete(url));

    return result === 'Ok';
  }

  async getPage(options?: { directory?: Key; nextKey?: Key }): Promise<KeyValuePage> {
    let url = this.url;

    if (options?.directory) {
      url = `${this.url}/base64:${encode(options.directory.key)}/`;
    }

    let params = {};
    if (options?.nextKey) {
      params = {
        from: `base64:${encode(options.nextKey.key)}`,
      };
    }

    const result = await this.request.send((token) => seaFetch(token).get(`${url}?` + new URLSearchParams(params)));

    return mapToKeyValuePage(result);
  }

  async getAllPages(options?: { directory?: Key; nextKey?: Key }): Promise<KeyValue[]> {
    const pages: KeyValue[] = [];
    let forNextKey = options?.nextKey;

    while (true) {
      const pageResult = await this.getPage({
        directory: options?.directory,
        nextKey: forNextKey,
      });

      const page: KeyValuePage = pageResult;
      pages.push(...page.keyValuePairs);

      if (page.nextKey) {
        forNextKey = page.nextKey;
      } else {
        return pages;
      }
    }
  }
}
