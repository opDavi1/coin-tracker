import Image from 'next/image'
import styles from './styles.module.css'

function Coin() {
  return (
    <div className={styles.coin}>
      <Image src="" alt="Coin" />
        <ul>
          <li><Image src="" alt="Flag" />Name</li>
          <li>year</li>
          <li>Type</li>
          <li>Composition * weight * diameter</li>
      </ul>
    </div>
  )
}

export default function Page() {
  return (
    <div>
      <Coin />   
      <Coin />   
      <Coin />   
    </div>  
  )
}
