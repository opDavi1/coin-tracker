import redirect from 'next/navigation';
import Coin from '../../coin.tsx';

export async function POST(request: Request) {
    const formData = await request.formData();
    if(formData?.name) {
        return new Response('No form data in request or coin name was not sent', {
          status: 400
        });
    }
    const coin: Coin = {
      id: 0,
      numistaId: formData.numistaId,
      name: formData.name,
      coinType: formData.coinType,
      issuer: formData.issuer,
      country: formData.country,
      minYear: formData.minYear,
      maxYear: formData.maxYear,
      composition: formData.composition,
      shape: formData.coinShape
    };

    console.log(coin);
    return new Response('Successfully added coin to the database', {
      status: 301,
      headers: {
        'Location': '/',
      },
    });
}
