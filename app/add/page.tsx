import styles from './styles.module.css';

export default function addCoin() {
    return (
        <div className={styles.addCoinDialogue}>
            <h1>Add a coin to the database:</h1>
            <form name="addCoin" action="/add" method="post" autoCapitalize="none" autoComplete="off">
                <label htmlFor="name">Name * </label>
                <input type="text" name="name" id="name" required /><br />

                <label htmlFor="numistaId">Numista ID</label>
                <input type="text" name="numistaId" id="numistaId" /><br />

                <label htmlFor="coinType">Type</label>
                <select name="coinType" id="coinType">
                    <option value="0">Standard Circulation Coins</option>
                    <option value="1">Circulating Commemorative Coins</option>
                    <option value="2">Non-Circulating Coins</option>
                    <option value="3">Collector Coins</option>
                    <option value="4">Siege Coins</option>
                    <option value="5">Official Necessity Coins</option>
                    <option value="6">Merchant Tokens</option>
                    <option value="7">Local Coins</option>
                    <option value="8">Patterns</option>
                    <option value="9">Contemporary Counterfeits</option>
                    <option value="10">Proto-Coins</option>
                    <option value="11">Other</option>
                </select><br />
                
                <label htmlFor="issuer">Issuer</label>
                <input type="text" name="issuer" id="issuer" /><br />
                
                <label htmlFor="country">Country</label>
                <input type="text" name="country" id="country" /><br />
                
                <label htmlFor="minYear">Min Year</label>
                <input type="text" name="minYear" id="minYear" /><br />

                <label htmlFor="maxYear">Max Year</label>
                <input type="text" name="maxYear" id="maxYear" /><br />
                
                <label htmlFor="composition">Composition</label>
                <input type="text" name="composition" id="composition" /><br />
                
                <label htmlFor="coinShape">Shape</label>
                <select name="coinShape" id="coinShape">
                    <option value="0">Round</option>
                    <option value="1">Square</option>
                    <option value="2">Polygonal</option>
                    <option value="3">Scalloped</option>
                    <option value="4">Triangular</option>
                    <option value="5">Other</option>
                </select><br />

                <label htmlFor="diameter">Diameter (mm)</label>
                <input type="number" name="diameter" id="diameter" step="0.01" /><br />

                <label htmlFor="thickness">Thickness (mm)</label>
                <input type="number" name="thickness" id="thickness" step="0.01" /><br />
                
                <label htmlFor="weight">Weight (g)</label>
                <input type="number" name="weight" id="weight" step="0.01" /><br />

                <label htmlFor="coinOrientation">Orientation</label>
                <select name="coinOrientation" id="coinOrientation">
                    <option value="0">Medal</option>
                    <option value="1">Coin</option>
                    <option value="2">Other</option>
                </select><br />
                
                <label htmlFor="denomination">Denomination</label>
                <input type="text" name="denomination" id="denomination" /><br />
                
                <label htmlFor="value">Value</label>
                <input type="number" name="value" id="value" step="0.01" /><br />

                <label htmlFor="valueNumerator">Value Numerator</label>
                <input type="number" name="valueNumerator" id="valueNumerator" step="1" /><br />
                
                <label htmlFor="valueDenominator">Value Denominator</label>
                <input type="number" name="valueDenominator" id="valueDenominator" step="1" /><br />
                
                <label htmlFor="currency">Currency</label>
                <input type="text" name="currency" id="currency" /><br />
                
                <label htmlFor="grade">Grade</label>
                <input type="number" name="grade" id="grade" min="0" max="70" step="5"/><br />
                
                <label htmlFor="isDemonitized">Demonetized?</label>
                <input type="checkbox" name="isDemonitized" id="isDemonitized" /><br />
                
                <label htmlFor="comments">Comments</label>
                <textarea name="comments" id="comments"></textarea><br /><br />
                
                <input type="submit" value="Add Coin"/>
            </form>
        </div>
    );
}