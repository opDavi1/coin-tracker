import React from 'react';
import Image from 'next/image';
import styles from './styles.module.css';
import {Coin, CoinType, CoinOrientation, CoinShape, coinTypeToString} from './coin.tsx';


const testCoin: Coin = {
  name: "Test coin",
  minYear: 2007,
  maxYear: 2025,
  coinType: CoinType.StandardCirculationCoins,
  composition: "Silver (.500)",
  weight: 6.68,
  diameter: 28.8,
  obverseImage: "./tauri.svg",
  id: 0,
  numistaId: 0,
  issuer: 'opDavi1',
  country: 'opDavi1',
  shape: CoinShape.Round,
  thickness: 0,
  orientation: CoinOrientation.Medal,
  denomination: '',
  value: 0,
  valueNumerator: 0,
  valueDenominator: 0,
  currency: '',
  grade: 70,
  reverseImage: '',
  obverseDescription: '',
  reverseDescription: '',
  isDemonitized: false,
  comments: 'This is a test coin'
};

function CoinListing({coin}: {coin: Coin}) {
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

function CoinContainer({coins}: {}) {
    if(!coins?.length) {
        return(
            <div>
                <h1>No Coins Found.</h1>
            </div>
        );
    }

    const coinListings = coins.map(coin => <CoinListing coin={coin}/>);
    return (
        <div>
            {coinListings}
        </div>
    );
}

export default function Page() {
  return (
    <CoinContainer coins={[testCoin, testCoin, testCoin]}/>
  );
}
