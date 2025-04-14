import React from 'react';
import {Coin, CoinContainer, CoinType, CoinOrientation, CoinShape} from './_components/coin/coin.tsx';


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
  issuer: 'opDavi1',
  country: 'opDavi1',
  shape: CoinShape.Round,
  thickness: 0,
  orientation: CoinOrientation.Medal,
  value: 0,
  valueNumerator: 0,
  valueDenominator: 0,
  grade: 70,
  isDemonitized: true,
  comments: 'This is a test coin'
};


export default function Page() {
  return (
    <CoinContainer coins={[testCoin, testCoin, testCoin]}/>
  );
}
