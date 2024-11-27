defmodule XmlParser do
  import SweetXml

  def main(_args) do
    parse()
  end

  def parse() do
    {:ok, content} = File.read "src/rezepte.xml"
    xml = SweetXml.parse(content)
  end
end
