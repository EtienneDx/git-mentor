const StudentsAPI = {
  fetch: async () => {
    return [
      {
        name: "Student A",
        id: "1",
        creationDate: new Date(2000, 1, 1),
      },
      {
        name: "Student B",
        id: "2",
        creationDate: new Date(2000, 1, 1),
      },
      {
        name: "Student C",
        id: "3",
        creationDate: new Date(2000, 1, 1),
      },
    ];
  },
};

export default StudentsAPI;
