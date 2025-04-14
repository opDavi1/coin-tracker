import React from 'react';
import Image from 'next/image';
import styles from './styles.module.css';

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
    isDemonetized: boolean,
    comments: string,
}


export enum CoinOrientation {
    Medal,
    Coin,
    Other,
}

export function coinOrientationToString(orientation: CoinOrientation): String {
    switch (orientation) {
        case CoinOrientation.Medal:
            return 'Medal';
            break;
        case CoinOrientation.Coin:
            return 'Coin';
            break;
        default:
            return 'Other';
            break;
    }
}


export enum CoinShape {
    Round,
    Square,
    Polygonal,
    Scalloped,
    Triangular,
    Other,
}

export function coinShapeToString(shape: CoinShape): String {
    switch (shape) {
        case CoinShape.Round:
            return 'Round';
            break;
        case CoinShape.Square:
            return 'Square';
            break;
        case CoinShape.Polygonal:
            return 'Polygonal';
            break;
        case CoinShape.Scalloped:
            return 'Scalloped';
            break;
        case CoinShape.Triangular:
            return 'Triangular';
            break;
        default:
            return 'Other';
            break;
    }
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

export function CoinListing({coin}: {coin: Coin}) {
  return (
    <div className={styles.coin}>
      <Image src={coin.obverseImage} alt="image" width={50} height={50}/>
        <ul>
          <li><Image src="" alt="Flag" />Test Coin</li>
          <li>{coin.minYear} - {coin.maxYear}</li>
          <li>{coinTypeToString(coin.coinType)}</li>
          <li>{coin.composition} • {coin.weight}g • ⌀ {coin.diameter} mm</li>
      </ul>
    </div>
  );
}

export function CoinContainer({coins}: {coins: Array<Coin>}) {
    if(!coins?.length) {
        return(
            <div>
                <h1>No Coins Found.</h1>
            </div>
        );
    }

    const coinListings = coins.map(coin => <CoinListing coin={coin}/>);
    return (
        <div className={styles.coinContainer}>
            {coinListings}
        </div>
    );
}
