const GroupsAPI = {
  fetch: async () => {
    return [
      {
        name: "Group A",
        id: "1",
        creationDate: new Date(2000, 1, 1),
      },
      {
        name: "Group B",
        id: "2",
        creationDate: new Date(2000, 1, 1),
      },
      {
        name: "Group C",
        id: "3",
        creationDate: new Date(2000, 1, 1),
      },
    ];
  },
};

export default GroupsAPI;
