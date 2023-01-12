import Configuration from '../configuration';
import Request from './request';
import seaFetch from './seaFetch';
import { Lock, Name, HeldLock, toLock, toHeldLock, LockPage, toLockPage } from '../model/locks';

import { encode } from '../utils/base64';

export default class Locks {
  url: string;
  request: Request;

  constructor(configuration: Configuration) {
    this.url = `${configuration.values().coordinationEndpoint}/locks`;
    this.request = new Request(configuration.identify);
  }

  async get(name: Name): Promise<Lock> {
    const url = `${this.url}/base64:${encode(name.name)}`;

    const result = await this.request.send((token) => seaFetch(token).get(url));

    return toLock(result);
  }

  async acquire(name: Name, clientId: string, ttl: number): Promise<HeldLock> {
    const url = `${this.url}/base64:${encode(name.name)}`;

    const params = {
      'client-id': clientId,
      ttl: String(ttl),
    };

    const result = await this.request.send((token) =>
      seaFetch(token).post(`${url}?` + new URLSearchParams(params), '{}'),
    );

    return toHeldLock(result);
  }

  async release(name: Name, id: string): Promise<boolean> {
    const url = `${this.url}/base64:${encode(name.name)}`;

    const params = {
      id: id,
    };

    const result = await this.request.send((token) => seaFetch(token).delete(`${url}?` + new URLSearchParams(params)));

    return result === 'OK';
  }

  async renew(name: Name, id: string, ttl: number): Promise<boolean> {
    const url = `${this.url}/base64:${encode(name.name)}`;

    const params = {
      id,
      ttl: String(ttl),
    };

    const result = await this.request.send((token) =>
      seaFetch(token).post(`${url}?` + new URLSearchParams(params), '{}'),
    );

    return result === 'OK';
  }

  async getPage(options?: { directory?: Name; fromLock?: Name }): Promise<LockPage> {
    let url = this.url;

    if (options?.directory) {
      url = `${this.url}/base64:${encode(options.directory.name)}/`;
    }

    let params = {};
    if (options?.fromLock) {
      params = {
        from: `base64:${encode(options.fromLock.name)}`,
      };
    }

    const result = await this.request.send((token) => seaFetch(token).get(`${url}?` + new URLSearchParams(params)));

    return toLockPage(result);
  }

  async getAllPages(options?: { directory?: Name; fromLock?: Name }): Promise<Lock[]> {
    const pages: Lock[] = [];
    let forNextLock = options?.fromLock;

    while (true) {
      const pageResult = await this.getPage({
        directory: options?.directory,
        fromLock: forNextLock,
      });

      const page: LockPage = pageResult;
      pages.push(...page.locks);

      if (page.nextLock) {
        forNextLock = page.nextLock;
      } else {
        return pages;
      }
    }
  }
}
