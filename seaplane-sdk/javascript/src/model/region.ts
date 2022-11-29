export enum Region {
  Asia = "XA",
  RepublicOfChina = "XC",
  Europe = "XE",
  Africa = "XF",
  NorthAmerica = "XN",
  Oceania = "XO",
  Antartica = "XQ",
  SouthAmerica = "XS",
  Uk = "XU"
}

export const mapToRegions = (regions?: string[]): Region[] => {
  if(!regions) {
    return []
  }

  return regions.map(region => Region[region])
}