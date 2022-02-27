import { render, screen} from "@testing-library/svelte";
import INDEX from "./index.svelte";

describe("INDEX", () => {
  test("works!", () => {
    render(INDEX);
    const node = screen.queryByText("In Sports News!");
    expect(node).toBeInTheDocument();
  });
});