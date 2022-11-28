import { decode } from '../utils/base64';
import { Provider } from './provider'
import { Region } from './region'
import { Key } from './metadata'

enum SeaplaneApi {
  Locks = "Locks",
  Metadata = "Config"
}

enum RestrictionState {
  Enforced = "Enforced",
  Pending = "Pending"
};

type RestrictionDetails = {
  regionsAllowed: [Region]
  regionsDenied: [Region]
  providersAllowed: [Provider]
  providersDenied: [Provider]
}

type Restriction = {
  api: SeaplaneApi
  directory: Key
  details: RestrictionDetails
  state: RestrictionState
}

type RestrictionPage = {
  restrictions: [Restriction]
  nextApi: SeaplaneApi | null
  nextKey: Key | null
}

export type LockInfo = {
  ttl: number;
  clientId: string;
  ip: string;
};

