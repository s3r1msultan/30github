import axios from "axios";

const API_URL = "http://127.0.0.1:8080";

export const getBoard = async () => {
  try {
    const res = await axios.get(`${API_URL}/board`);
    return res.data.board_fen;
  } catch (error) {
    console.error("Error fetching board:", error);
    return "start";
  }
};

export const makeMove = async (move: string) => {
  try {
    const res = await axios.post(`${API_URL}/move`, { chess_move: move });
    return res.data.board_fen;
  } catch (error) {
    console.error("Error making move:", error);
    throw new Error("Invalid move");
  }
};
