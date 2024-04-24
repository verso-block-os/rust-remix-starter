import { parse } from "cookie";
import { api } from "./api";

export const getSession = async (cookies: string | null) => {
  const token = parse(cookies || "").auth_session;

  if (!token) {
    return null;
  }

  try {
    return await api.query(["auth.verify", token]);
  } catch {
    return null;
  }
};
