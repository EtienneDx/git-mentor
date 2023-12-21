import { useParams } from "react-router-dom";

const GroupStudents = () => {
  const { groupId } = useParams();

  return <h1>Student List for Group {groupId}</h1>;
};

export default GroupStudents;
