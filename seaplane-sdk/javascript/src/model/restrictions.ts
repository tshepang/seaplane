import { Provider, mapToProvider } from './provider'
import { Region, mapToRegions } from './region'
import { Key, mapKey } from './metadata'
import {SeaplaneError} from './errors'

export enum SeaplaneApi {
  Locks,
  Metadata
}

export enum RestrictionState {
  Enforced = "Enforced",
  Pending = "Pending"
};

export type RestrictionDetails = {
  regionsAllowed: Region[]
  regionsDenied: Region[]
  providersAllowed: Provider[]
  providersDenied: Provider[]
}

export type Restriction = {
  api: SeaplaneApi
  directory: Key
  details: RestrictionDetails
  state: RestrictionState
}

export type RestrictionPage = {
  restrictions: Restriction[]
  nextApi: SeaplaneApi | null
  nextKey: Key | null
}

export type LockInfo = {
  ttl: number;
  clientId: string;
  ip: string;
};

export const mapToRestriction = (restriction: object): Restriction => {
  console.log("Restriction: ", restriction)
  const key = mapKey(restriction["directory"])

  if (key == null) {
    throw new SeaplaneError("Directory must not be null")
  }
  
  console.log("Buenos dias", RestrictionState[capitalize(restriction["state"])])

  return {
    api: seaplaneApi(restriction["api"]),
    directory: key,
    details: mapToRestrictionDetails(restriction),
    state: RestrictionState[capitalize(restriction["state"])]
  }
}

const mapToRestrictionDetails =  (restriction: object): RestrictionDetails => ({
  regionsAllowed: mapToRegions(restriction["regions_allowed"]),
  regionsDenied: mapToRegions(restriction["regions_denied"]),
  providersAllowed: mapToProvider(restriction["providers_allowed"]),
  providersDenied: mapToProvider(restriction["providers_denied"])
})

const mapToSeaplaneApi = (api?: string) : SeaplaneApi | null => {
  if(!api) {
    return null
  }

  return SeaplaneApi[capitalize(api)]
}

export const mapToRestrictionPage = (restrictionPage: object): RestrictionPage => ({
  restrictions: restrictionPage["restrictions"].map(mapToRestriction),
  nextApi: mapToSeaplaneApi(restrictionPage["next_api"]),
  nextKey: mapKey(restrictionPage["next_key"])
})

const capitalize: (string: string) => string = (string) => string.charAt(0).toUpperCase() + string.slice(1);

const seaplaneApi: (api: string) => SeaplaneApi = (api) => api  == "config" ? SeaplaneApi.Metadata : SeaplaneApi.Locks
