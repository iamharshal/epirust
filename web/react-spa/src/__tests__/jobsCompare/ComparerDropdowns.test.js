/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

import React from "react";
import ShallowRenderer from 'react-test-renderer/shallow';
import ComparerDropdowns from "../../jobsCompare/ComparerDropdowns";

describe('Comparer Dropdowns', () => {
  it('should render inputs to select jobs to compare', () => {
    const renderer = new ShallowRenderer()
    renderer.render(
      <ComparerDropdowns jobs={[{_id: 'id1'}, {_id: 'id2'}]}/>
    );

    expect(renderer.getRenderOutput()).toMatchSnapshot()
  });
});

