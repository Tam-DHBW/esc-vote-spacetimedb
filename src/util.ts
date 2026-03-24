import type { RoundKind } from "./module_bindings/types";

export const formatRoundKind = (tag: RoundKind["tag"]): string =>
  tag
    .replace(/([A-Z])/g, " $1")
    .replace(/(\d+)/g, " $1")
    .trim();
