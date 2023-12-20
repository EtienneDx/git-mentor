import { ReactNode } from "react";

const Box = ({ children }: { children: ReactNode }) => {
  return (
    <div className="flex flex-col gap-4 border border-black rounded-lg p-8 bg-white">
      {children}
    </div>
  );
};

export default Box;
