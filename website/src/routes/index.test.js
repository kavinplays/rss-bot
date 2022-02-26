import { render, screen} from "@testing-library/svelte";
import INDEX from "./index.svelte";

describe("INDEX", () => {
    test("says 'hello world!'", () => {
      render(INDEX);
      const node = screen.queryByText("power users");
    //   expect(node).toBeInTheDocument();
    });
  });