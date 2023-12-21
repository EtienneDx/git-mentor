type ColumnHeaderProps = {
  title: string;
  width?: "small" | "medium" | "large";
};

export const ColumnHeader = ({ title, width }: ColumnHeaderProps) => {
  const thClassSmall =
    "px-4 py-2 text-left text-gray-600 border-gray-300 border-x w-1/12";
  const thClassMedium =
    "px-4 py-2 text-left text-gray-600 border-gray-300 border-x w-1/6";
  let thClass = "px-4 py-2 text-left text-gray-600 border-gray-300 border-x";
  switch (width) {
    case "small":
      thClass = thClassSmall;
      break;
    case "medium":
      thClass = thClassMedium;
      break;
  }

  return <th className={thClass}>{title}</th>;
};
