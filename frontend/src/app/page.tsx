import { Header } from "@/components/Header";
import { TokenBalances } from "@/components/TokenBalances";
import { Sweep } from "@/components/Sweep";
import { Swap } from "@/components/Swap";

export default function Home() {
  return (
    <div className="flex flex-col min-h-screen bg-gray-900 text-white">
      <Header />
      <main className="flex-1 flex flex-col items-center p-8">
        <TokenBalances />
        <Sweep />
        <Swap />
      </main>
    </div>
  );
}
