import { useParams } from "react-router-dom";

const GroupAssignements = () => {
  const { groupId } = useParams();

  return <h1>Assignements List for Group {groupId}</h1>;
};

export default GroupAssignements;
