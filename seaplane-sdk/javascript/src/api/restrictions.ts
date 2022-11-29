import Configuration from '../configuration';
import Request, { headers } from './request';

import { encode } from '../utils/base64';
import { Key } from '../model/metadata';
import { SeaplaneApi, Restriction, mapToRestriction, RestrictionDetails, RestrictionPage, mapToRestrictionPage } from '../model/restrictions'
import { SeaplaneError } from '../model/errors';

const axios = require('axios'); // eslint-disable-line

export default class Restrictions {
  url: string;
  request: Request;

  constructor(configuration: Configuration) {
    this.url = `${configuration.values().coordinationEndpoint}/restrict`;
    this.request = new Request(configuration.identify);
  }

  async get(api: SeaplaneApi, key: Key): Promise<Restriction> {
    const url = `${this.url}/base64:${encode(key.key)}`;

    const result = await this.request.send((token) =>
      axios.get(url, {
        headers: headers(token),
      }),
    );

    return mapToRestriction(result);
  }

  async set(api: SeaplaneApi, key: Key, restrictionDetails: RestrictionDetails): Promise<boolean> {
    const url = `${this.url}/${api}/base64:${encode(key.key)}`;
    
    const data = {
      "regions_allowed": restrictionDetails.regionsAllowed.map(region => String(region)),
      "regions_denied": restrictionDetails.regionsDenied.map(region => String(region)),
      "providers_allowed": restrictionDetails.providersAllowed.map(provider => String(provider)),
      "providers_denied": restrictionDetails.providersDenied.map(provider => String(provider))
    };

    const result = await this.request.send((token) =>
      axios.post(url, data, {
        headers: headers(token)        
      }),
    );

    return result == "Ok";
  }

  async delete(api: SeaplaneApi, key: Key): Promise<boolean> {
    const url = `${this.url}/${api}/base64:${encode(key.key)}`;

    const result = await this.request.send((token) =>
      axios.delete(url, {
        headers: headers(token)        
      }),
    );

    return result === 'Ok';
  }

  async getPage(options?: { 
    api?: SeaplaneApi,
    fromRestriction?: Key,
    isAllRange?: boolean
  }): Promise<RestrictionPage> {
    let url = this.url;    
    
    if (options?.api && !options?.isAllRange) {
      url = `${url}/${options!.api}`;
    }

    let params = {};
    if (options?.fromRestriction) {
      if(!options?.api) {
        throw new SeaplaneError("You must set 'api' with 'fromRestriction' parameters.")
      }

      params = {
        from: `base64:${encode(options.fromRestriction.key)}`,
        from_api: String(options.api)
      };
    }

    const result = await this.request.send((token) =>
      axios.get(url, {
        headers: headers(token),
        params: params,
      }),
    );

    return mapToRestrictionPage(result);
  }

  async getAllPages(options?: { 
    api?: SeaplaneApi,
    fromRestriction?: Key    
  }): Promise<Restriction[]> {
    const pages: Restriction[] = [];
    let forNextRestriction = options?.fromRestriction;
    let fromApi = options?.api

    while (true) {
      const pageResult = await this.getPage({
        api: fromApi,
        fromRestriction: forNextRestriction,
        isAllRange: true
      });

      const page: RestrictionPage = pageResult;
      pages.push(...page.restrictions);

      if (page.nextKey && page.nextApi) {
        forNextRestriction = page.nextKey;
        fromApi = page.nextApi
      } else {
        return pages;
      }
    }
  }
}
