export type Coin = {
    id: number,
    numistaId: number,
    name: string,
    coinType: CoinType,
    issuer: string,
    country: string,
    minYear: number,
    maxYear: number,
    composition: string,
    shape: CoinShape,
    diameter: number,
    thickness: number,
    weight: number,
    orientation: CoinOrientation,
    denomination: string,
    value: number,
    valueNumerator: number,
    valueDenominator: number,
    currency: string,
    grade: number,
    obverseImage: string,
    reverseImage: string,
    obverseDescription: string,
    reverseDescription: string,
    isDemonitized: boolean,
    comments: string,
}


export enum CoinOrientation {
    Medal,
    Coin,
    Other,
}

export enum CoinShape {
    Round,
    Square,
    Polygonal,
    Scalloped,
    Triangular,
    Other,
}

export enum CoinType {
    StandardCirculationCoins,
    CirculatingCommemorativeCoins,
    NonCirculatingCoins,
    CollectorCoins,
    SiegeCoins,
    OfficialNecessityCoins,
    MerchantTokens,
    LocalCoins,
    Patterns,
    ContemporaryCounterfeits,
    ProtoCoins,
    Other,
}

export function coinTypeToString(t: CoinType): String {
    switch (t) {
        case CoinType.StandardCirculationCoins:
            return "Standard Circulation Coins";
            break;
        case CoinType.CirculatingCommemorativeCoins:
            return "Circulating Commemorative Coins";
            break;
        case CoinType.NonCirculatingCoins:
            return "Non Circulating Coins";
            break;
        case CoinType.CollectorCoins:
            return "Collector Coins"
            break;
        case CoinType.SiegeCoins:
            return "Siege Coins";
            break;
        case CoinType.OfficialNecessityCoins:
            return "Official Necessity Coins";
            break;
        case CoinType.MerchantTokens:
            return "Merchant Tokens";
            break;
        case CoinType.LocalCoins:
            return "Local Coins";
            break;
        case CoinType.Patterns:
            return "Patterns";
            break;
        case CoinType.ContemporaryCounterfeits:
            return "Contemporary Counterfeits";
            break;
        case CoinType.ProtoCoins:
            return "Proto-Coins";
            break;
        default:
            return "Other"
            break;
    }
}