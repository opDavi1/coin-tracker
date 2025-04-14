import React from "react"
import './globals.css'
import Navbar from './_components/navbar/navbar.tsx'

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
      </head>
      <body>
      <Navbar />
      {children}
      </body>
    </html>
  )
}
