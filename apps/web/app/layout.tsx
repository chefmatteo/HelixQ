import type { Metadata } from "next";
import { Inter, IBM_Plex_Mono } from "next/font/google";
import "./globals.css";
import "./raycast.css";
import ScrollRestoration from "./ScrollRestoration";

const inter = Inter({
  subsets: ["latin"],
  weight: ["400", "500", "600", "700"],
  display: "swap",
});

const ibmPlexMono = IBM_Plex_Mono({
  subsets: ["latin"],
  weight: ["400", "500"],
  display: "swap",
  variable: "--font-mono",
});

export const metadata: Metadata = {
  title: {
    absolute: "HelixQ — YouTube metadata search",
  },
  description:
    "HelixQ searches YouTube videos by title, description, and engagement signals.",
  icons: {
    icon: [{ url: "/favicon.png", type: "image/png" }],
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="scroll-smooth h-full">
      <body
        className={`antialiased h-full ${inter.className} ${ibmPlexMono.variable}`}
      >
        <ScrollRestoration />
        {children}
      </body>
    </html>
  );
}
