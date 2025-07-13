import Select from "../../components/Select";

function CharacterInfo({ editedStats, setEditedStats }) {
  const voice = ["Young Voice", "Mature Voice", "Aged Voice"];
  const gender = ["Female", "Male"];
  const origins = [
    "Milquetoast",
    "Lone Survivor",
    "Troubled Childhood",
    "Violent Past",
    "Professional",
    "Military Veteran",
    "Noble Scion",
    "Cruel Fate",
    "Waste of Skin",
  ];
  const ng = ["NG0", "NG+1", "NG+2", "NG+3", "NG+4", "NG+5", "NG+6", "NG+7"];

  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-evenly",
        marginTop: "5px",
      }}
    >
      <Select
        name={"Gender"}
        options={gender}
        setEditedStats={setEditedStats}
        editedStats={editedStats}
      />
      <Select
        name={"Origin"}
        options={origins}
        setEditedStats={setEditedStats}
        editedStats={editedStats}
      />
      <Select
        name={"Voice"}
        options={voice}
        setEditedStats={setEditedStats}
        editedStats={editedStats}
      />
      <Select
        name={"Ng"}
        options={ng}
        setEditedStats={setEditedStats}
        editedStats={editedStats}
      />
    </div>
  );
}

export default CharacterInfo;
