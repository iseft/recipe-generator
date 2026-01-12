import { useEffect } from "react";
import { useAuth } from "@clerk/clerk-react";
import { setAuthTokenGetter } from "../../api/api-client";

export function useAuthSetup() {
  const { getToken } = useAuth();

  useEffect(() => {
    setAuthTokenGetter(async () => {
      try {
        return await getToken();
      } catch (error) {
        console.error("Failed to get auth token:", error);
        return null;
      }
    });
  }, [getToken]);
}
