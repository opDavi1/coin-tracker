import React from "react"

export const metadata = {
  title: 'Coin tracker',
  description: 'track ur coins',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <head>
        <meta charSet="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Coin Tracker</title>
      </head>
      <body>{children}</body>
    </html>
  )
}
