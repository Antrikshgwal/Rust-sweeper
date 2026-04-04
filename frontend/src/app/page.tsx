"use client";
import { Header } from "@/components/Header";
import { TokenBalances } from "@/components/TokenBalances";
import { Sweep } from "@/components/Sweep";
import { Swap } from "@/components/Swap";
import { useWallet } from "@/contexts/WalletContext";

function Landing() {
  const { connect } = useWallet();

  return (
    <div className="flex-1 flex flex-col items-center justify-center px-8 text-center">
      <h2 className="text-5xl font-extrabold mb-4 bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
        Dust Sweeper
      </h2>
      <p className="text-gray-400 text-lg max-w-xl mb-2">
        Convert scattered token dust into a single token with one click.
        Sweep small ERC-20 balances across your wallet into USDC, USDT, or WETH.
      </p>
      <p className="text-gray-500 text-sm mb-8">Sepolia Testnet</p>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-3xl w-full mb-10">
        <div className="bg-gray-800 rounded-lg p-5">
          <div className="text-3xl mb-3">1</div>
          <h3 className="font-semibold mb-1">Connect</h3>
          <p className="text-gray-400 text-sm">Link your MetaMask wallet on Sepolia testnet</p>
        </div>
        <div className="bg-gray-800 rounded-lg p-5">
          <div className="text-3xl mb-3">2</div>
          <h3 className="font-semibold mb-1">Review</h3>
          <p className="text-gray-400 text-sm">See all your token balances and dust amounts</p>
        </div>
        <div className="bg-gray-800 rounded-lg p-5">
          <div className="text-3xl mb-3">3</div>
          <h3 className="font-semibold mb-1">Sweep</h3>
          <p className="text-gray-400 text-sm">Consolidate everything into your chosen token</p>
        </div>
      </div>

      <button
        onClick={connect}
        className="px-8 py-3 bg-blue-500 rounded-lg text-lg font-semibold hover:bg-blue-600 transition-colors"
      >
        Connect Wallet
      </button>
    </div>
  );
}

export default function Home() {
  const { address } = useWallet();

  return (
    <div className="flex flex-col min-h-screen bg-gray-900 text-white">
      <Header />
      {address ? (
        <main className="flex-1 flex flex-col items-center p-8">
          <TokenBalances />
          <Sweep />
          <Swap />
        </main>
      ) : (
        <Landing />
      )}
    </div>
  );
}
