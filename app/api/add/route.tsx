import { Coin } from '../../_components/coin/coin.tsx';

/*
const schema = zfd.formData({
  numistaId: zfd.numeric().optional(),
  name: zfd.text(),
  coinType: zfd.numeric().optional(),
})
*/

export async function POST(request: Request) {
  const formData = await request.formData();
  if (!formData?.get('name')) {
    return new Response('400: Bad request\nMalformed form data', {
      status: 400,
    });
  }

  const coin: Coin = {
    id: -1,
    numistaId: Number(formData.get('numistaId')),
    name: String(formData.get('name')),
    coinType: Number(formData.get('coinType')),
    issuer: String(formData.get('issuer')),
    country: String(formData.get('country')),
    minYear: Number(formData.get('minYear')),
    maxYear: Number(formData.get('maxYear')),
    composition: String(formData.get('composition')),
    shape: Number(formData.get('coinShape')),
    diameter: Number(formData.get('diameter')),
    thickness: Number(formData.get('thickness')),
    weight: Number(formData.get('weight')),
    orientation: Number(formData.get('orientation')),
    denomination: String(formData.get('denomination')),
    value: Number(formData.get('value')),
    valueNumerator: Number(formData.get('valueNumerator')),
    valueDenominator: Number(formData.get('valueDenominator')),
    currency: String(formData.get('currency')),
    grade: Number(formData.get('grade')),
    obverseImage: String(formData.get('obverseImage')),
    reverseImage: String(formData.get('reverseImage')),
    obverseDescription: String(formData.get('obverseDescription')),
    reverseDescription: String(formData.get('reverseDescription')),
    isDemonetized: Boolean(formData.get('isDemonetized')),
    comments: String(formData.get('comments')),
  };

  return new Response('Successfully added coin to the database', {
    status: 301,
    headers: {
      'Location': '/',
    },
  });
}
