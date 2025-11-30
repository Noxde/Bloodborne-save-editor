import Select from "../../components/Select";

function CharacterInfo({ editedStats, setEditedStats }) {
  const voice = [
    // Normal voices
    { label: "Young Voice", value: 0 },
    { label: "Mature Voice", value: 1 },
    { label: "Aged Voice", value: 2 },
    // NPCs voices
    { label: "Simon", value: 41 },
    { label: "Valtr", value: 40 },
    { label: "Brador", value: 42 },
    { label: "Annalise", value: 17 },
    { label: "Imposter doctor", value: 24 },
    { label: "Bigoted old man", value: 25 },
    { label: "Lonely old woman", value: 26 },
    { label: "Beggar", value: 27 },
    { label: "Arianna", value: 28 },
    { label: "Adella", value: 32 },
    { label: "Alfred", value: 34 },
    { label: "Eileen", value: 35 },
    { label: "Djura", value: 33 },
    { label: "Micolash", value: 21 },
  ];
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
