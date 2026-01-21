import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";
import { ThemeProvider } from "@/components/theme-provider";
import { ModeToggle } from "@/components/ui/mode-toggle";
import Link from "next/link";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Pokémon Alternative Metas",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body
        className={`
          ${geistSans.variable} ${geistMono.variable}
          antialiased
          min-h-screen
        `}
      >
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <header className="h-16 border-b bg-background/50 backdrop-blur">
            <div className="h-full flex items-center justify-between px-6">
              <Link
                href="/"
                className="font-semibold text-lg hover:opacity-80 transition"
              >
                PAM
              </Link>
              <ModeToggle />
            </div>
          </header>

          <main className="max-w-7xl mx-auto px-4 py-4">{children}</main>
        </ThemeProvider>
      </body>
    </html>
  );
}
