import ChessGame from "../components/ChessBoard";

export default function Home() {
  return (
    <main className="flex flex-col items-center justify-center h-screen">
      <h1 className="text-2xl font-bold mb-4">Rusty Chess</h1>
      <ChessGame />
    </main>
  );
}
