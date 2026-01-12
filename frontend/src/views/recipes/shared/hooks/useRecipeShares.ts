import { useQuery } from "@tanstack/react-query";
import { axiosInstance } from "../../../../shared/api/api-client";

export interface Share {
  userId: string;
  email: string;
  createdAt: string;
}

export function useRecipeShares(recipeId: string | undefined) {
  return useQuery({
    queryKey: ["recipe", recipeId, "shares"],
    queryFn: async () => {
      const response = await axiosInstance.get<Share[]>(
        `/api/recipes/${recipeId}/shares`
      );
      return response.data;
    },
    enabled: !!recipeId,
  });
}
