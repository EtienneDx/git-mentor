import { ReactNode } from "react";

export const ColumnContent = ({ content }: { content: ReactNode }) => {
  return <td className="px-4 py-2 text-gray-800 truncate">{content}</td>;
};
