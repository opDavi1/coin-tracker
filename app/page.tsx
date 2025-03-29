import React from 'react'
import Image from 'next/image'
import styles from './styles.module.css'
import {Coin, CoinType, CoinOrientation, CoinShape, coinTypeToString} from './coin.tsx'


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
  issuer: '',
  country: '',
  shape: CoinShape.Round,
  thickness: 0,
  orientation: CoinOrientation.Medal,
  denomination: '',
  value: 0,
  valueNumerator: 0,
  valueDenominator: 0,
  currency: '',
  grade: 0,
  reverseImage: '',
  obverseDescription: '',
  reverseDescription: '',
  isDemonitized: false,
  comments: ''
}

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
  )
}

/* function CoinListing({coin}: {coin: Coin}) {
  return (
    <div className={styles.coin}>
      <Image src="" alt="image" />
        <ul>
          <li><Image src="" alt="Flag" />Test Coin</li>
          <li>2025 - 2025</li>
          <li>Coins &gt; Standard Circulation Coins</li>
          <li>Silver (.500) • 6.68g • ⌀ 28.8 mm</li>
      </ul>
    </div>
  )
} */

/* function CoinContainer({coin}: IProps) {
    let innerHtml = ""
    for(coin in coins) {
        innerHtml += `<CoinListing
            name={coin.name}
            yearMin={coin.yearMin}
            yearMax={coin.yearMax}
            type={coin.type}
            composition={coin.composition}
            weight={coin.weight}
            diameter={coin.diameter}
            flagUrl={coin.flagUrl}
            imageUrl={coin.flagUrl}
        \>`
    }
    return (
        <div>
        {innerHtml}
        </div>
    )
} */

export default function Page() {
  return (
    <div>
      <CoinListing coin={testCoin}/>
      <CoinListing coin={testCoin}/>
      <CoinListing coin={testCoin}/>
      <CoinListing coin={testCoin}/>
    </div>  
  )
}
