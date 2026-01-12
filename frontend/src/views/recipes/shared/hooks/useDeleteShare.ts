import { useMutation, useQueryClient } from "@tanstack/react-query";
import { axiosInstance } from "../../../../shared/api/api-client";

export function useDeleteShare(recipeId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (userId: string) => {
      await axiosInstance.delete(`/api/recipes/${recipeId}/shares/${userId}`);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["recipe", recipeId, "shares"],
      });
      queryClient.invalidateQueries({ queryKey: ["recipe", recipeId] });
    },
  });
}
