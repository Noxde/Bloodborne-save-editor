function About() {
  return (
    <section className="about-page">
      <h1>About</h1>
      <p><a href="https://github.com/Noxde/Bloodborne-save-editor" target="_blank" rel="noreferrer">Noxde’s Bloodborne Save Editor repository</a></p>
      <p><a href="https://ko-fi.com/noxde" target="_blank" rel="noreferrer">Support Noxde on Ko-fi</a></p>
      <p aria-hidden="true">&nbsp;</p>
      <p>Lamp controls contributed by <strong>Argagaes</strong>.</p>
      <p><a href="https://bb-save-editor.pages.dev/" target="_blank" rel="noreferrer">Bloodborne Save Editor website</a></p>
      <p><a href="https://github.com/Argagaes/bloodborne-save-editor-browser" target="_blank" rel="noreferrer">Argagaes’ browser editor repository</a></p>
      <small>Version {__APP_VERSION__}</small>
    </section>
  );
}

export default About;
