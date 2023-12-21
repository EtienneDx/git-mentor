import { useState, useEffect } from "react";
import StudentsAPI from "../services/studentsAPI";
import { Student } from "../../../helpers/types";

const StudentsList = () => {
  const [students, setStudents] = useState<Student[]>([]);

  useEffect(() => {
    const loadStudents = async () => {
      const newStudents: Student[] = await StudentsAPI.fetch();
      setStudents(newStudents);
    };
    loadStudents();
  }, []);

  return (
    <div>
      <h1>My Students</h1>
      {students.length === 0 ? (
        <h1>Loading Students...</h1>
      ) : (
        <div>
          {students.map((student) => (
            <div key={student.id}>{student.name}</div>
          ))}
        </div>
      )}
    </div>
  );
};

export default StudentsList;
