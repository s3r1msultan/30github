"use client";

import { useEffect, useState } from "react";
import { Chessboard } from "react-chessboard";
import { getBoard, makeMove } from "../utils/api";

export default function ChessGame() {
  const [fen, setFen] = useState("start");

  useEffect(() => {
    getBoard().then(setFen);
  }, []);

  const onDrop = (sourceSquare: string, targetSquare: string) => {
    const moveStr = `${sourceSquare}${targetSquare}`;

    makeMove(moveStr)
      .then((newFen) => {
        setFen(newFen);
      })
      .catch(() => {
        alert("Invalid move");
      });

    return true;
  };

  return (
    <div className="flex justify-center items-center h-screen min-w-96">
      <Chessboard
        position={fen}
        onPieceDrop={onDrop}
      />
    </div>
  );
}
