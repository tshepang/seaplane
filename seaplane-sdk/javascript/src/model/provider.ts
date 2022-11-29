export enum Provider {
  AWS = "AWS",
  Azure = "AZURE",
  DigitalOcean = "DIGITALOCEAN",
  Equinix = "EQUINIX",
  GCP = "GCP"
}

export const mapToProvider = (providers?: string[]): Provider[]  => {
  if (!providers) {
    return []
  }

  return providers.map(provider => Provider[provider])
}