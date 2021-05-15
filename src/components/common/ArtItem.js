import React from 'react';
import PropTypes from 'prop-types';
import styled from 'styled-components';

import Button from './Button';

import placeholderDataUrl from '../../assets/art.png';

const placeholderBid = 55;

const StyledContainer = styled('div')`
  position: relative;
  width: fit-content;
  margin: 15px 5px;
  transition: 250ms;

  :hover {
    transform: scale(1.01);
  }

  img {
    border-radius: 8px;
    max-width: 100%;

    @media (min-width: 1100px) {
      max-width: 320px;
    }
  }

  button {
    position: absolute;
    right: 20px;
    bottom: 20px;
  }
`;

const ArtItem = ({ dataUrl, bid }) => (
  <StyledContainer>
    <img src={dataUrl || placeholderDataUrl} alt="art" />
    <Button isPrimary isSmall>
      Bid {bid || placeholderBid}Ⓝ
    </Button>
  </StyledContainer>
);

ArtItem.propTypes = {
  dataUrl: PropTypes.string,
  bid: PropTypes.string,
};

export default ArtItem;
